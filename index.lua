local col = { 
colors.white,
colors.orange,
colors.magenta,
colors.lightBlue,
colors.yellow,
colors.lime,
colors.pink,
colors.gray,
colors.lightGray,
colors.cyan,
colors.purple,
colors.blue,
colors.brown,
colors.green,
colors.red,
colors.black
}

local monitor = peripheral.find("monitor")

local function separateByCharacter(line, sep) 
    if sep == nil then
        sep = "%s"
    end
    local t = {}
    
    for str in string.gmatch(line, "([^"..sep.."]+)") do
        table.insert(t, str)
    end
    return t
end

local baseColors = {
    0xF0F0F0,
    0xF2B233,
    0xE57FD8,
    0x99B2F2,
    0xDEDE6C,
    0x7FCC19,
    0xF2B2CC,
    0x4C4C4C,
    0x999999,
    0x4C99B2,
    0xB266E5,
    0x3366CC,
    0x7F664C,
    0x57A64E,
    0xCC4C4C,
    0x111111
}

local function changeColors(_colors)
    for i=1, 16 do
        if _colors[i] ~= nil then
            -- print(string.sub(_colors[i], 1, 8))
            -- print(#_colors)
            monitor.setPaletteColor(col[i], tonumber(_colors[i])) -- tonumber()
        end
    end   
end

local function setColors(string)
    -- print("changed colors")
    local colors = separateByCharacter(string, ";")
    -- print(colors[1])
    changeColors(colors)
end

FrameBuffer = {}
              --this is a bad implementation but it should do (3 hours of footage @ 60fps is only ~650,000 frames, @ 128bytes/frame its only ~8MB of ram) // this calculation has proven to be wildly incorrect, without any compression it's 154kb -- update: 37kb

function FrameBuffer.new()
    return {first = 0, last = -1}
end

FrameBuffer = FrameBuffer.new()

function FrameBuffer.push(value)
    local last = FrameBuffer.last + 1
    FrameBuffer.last = last
    FrameBuffer[last] = value
  end

function FrameBuffer.pop()
    local first = FrameBuffer.first
    if first > FrameBuffer.last then error("list is empty") end
    local value = FrameBuffer[first]
    FrameBuffer[first] = nil        -- to allow garbage collection
    FrameBuffer.first = first + 1
    return value
end

function FrameBuffer.isEmpty()
    local first = FrameBuffer.first
    if first > FrameBuffer.last then return true else return false end
end

monitor.setTextScale(0.5)
term.clear()
-- term.setCursorPos(1, 1)

function parseFrames(str)
    local frames = separateByCharacter(str, "=")
    for i=1,#frames do
        FrameBuffer.push(frames[i])
    end
end

function drawRow(str, color_foreground, color_background, pos_y)
    if pos_y<0 or pos_y>60 then
        -- return
        error("unable to draw pixel: oob on y axis - cannot draw at " .. pos_x .. ", *" ..pos_y .. "*")
    end

    monitor.setCursorPos(3, pos_y+3)
    
    -- print(string.len(str)..", "..string.len(color_foreground))
    monitor.blit(str, color_foreground, color_background) 
end

local function redrawScreen(str)

    local strings = separateByCharacter(str, ";")

    local startTime = os.epoch("utc")

        for y=1,60 do
            components = separateByCharacter(strings[y], ",")
            drawRow(string.sub(components[1], 1, 160), components[2], components[3], y)
        end

    local endTime = os.epoch("utc")

    print("drawn in " .. endTime-startTime .. "ms")
end

local is_eof = true

local function connectToWS()
    ws = assert(http.websocket("127.0.0.1:3000"))
    Frametime = ws.receive()
    print("running animation @ " .. Frametime .. "fps")
    -- Framecount = ws.receive()
    print(Framecount)
    Framecount = tonumber(Framecount)
    print(Framecount)
    os.queueEvent("player_start")
    while true do
        -- print("loading frames")
        local response = ws.receive()
        if response == "eof" then
            -- ws.close()
            is_eof = false
            print("websocket closed")
            break
        else
            -- local frames = separateByCharacter(response, "#") 
            -- print("Pushing frame into framebuffer")
            -- print(response)
            parseFrames(response)
            
            
            -- print(FrameBuffer.pop())
            -- setFrame(FrameBuffer.pop())
        end
        -- os.startTimer(0)
        -- os.pullEvent("timer")
        coroutine.yield()
    end
end

-- start of test

local Frametime = 24


function pushFrame()
    os.pullEvent("player_start")

    local fps = 24
    local frameDuration = 1000 / fps -- ~41.666ms
    local nextFrameTime = os.epoch("utc")
    
    
    framec = 0
    
    print("Started mainloop --------------------------------------")
    
    while is_eof do
        while not FrameBuffer.isEmpty() do
            local now = os.epoch("utc")
            
            if now >= nextFrameTime then
                print("drawing frame")
                -- Draw frame
                local frameComponents = separateByCharacter(FrameBuffer.pop(), "|")
                setColors(frameComponents[1])
                redrawScreen(frameComponents[2])
                
                local frameLength = os.epoch("utc") - now
                if frameLength > frameDuration then
                    print("WARN: DRAWING FRAME TOOK TOO LONG")
                end
                
                print("Frametime: " .. frameLength .. "ms")
                
                framec = framec + 1
                nextFrameTime = nextFrameTime + frameDuration
                
                if os.epoch("utc") > nextFrameTime + frameDuration then
                    nextFrameTime = os.epoch("utc") + frameDuration
                end
            else
                local waitTime = nextFrameTime - os.epoch("utc")
                
                if waitTime > 20 then
                    os.startTimer(0)
                    os.pullEvent("timer")
                end
                
                while os.epoch("utc") < nextFrameTime do end
            end
        end
        os.startTimer(0)
        os.pullEvent("timer")
    end


    ws.close()
    print("finished video")
end
-- print(FrameBuffer.pop())

local startTimeMeasure = os.epoch("utc")

parallel.waitForAll(connectToWS, pushFrame)

local endTimeMeasure = os.epoch("utc")
print("finished in " .. endTimeMeasure-startTimeMeasure .. "ms")

-- end of test