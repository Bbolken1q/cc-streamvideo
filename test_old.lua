local col = { colors.white,
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

local monitor = peripheral.find("monitor")

monitor.setTextScale(0.5)
term.clear()
-- term.setCursorPos(1, 1)

local function changeColors(_colors)
    for i=1, 15 do
        if _colors[i] ~= nil then
            monitor.setPaletteColor(col[i], tonumber(_colors[i]))
        end
    end   
end

-- local newColor = {
--     0xe5e5e5,
--     0xd8d8d8,
--     0xcccccc,
--     0xbfbfbf,
--     0xb2b2b2,
--     0xa6a6a6,
--     0x999999,
--     0x8c8c8c,
--     0x808080,
--     0x737373,
--     0x666666,
--     0x595959,
--     0x4c4c4c,
--     0x404040,
--     0x333333,
--     0x262626
--     }


-- changeColors(newColor)


FrameBuffer = {}
              --this is a bad implementation but it should do (3 hours of footage @ 60fps is only ~650,000 frames, @ 128bytes/frame its only ~8MB of ram) // this calculation has proven to be wildly incorrect, without any compression it's 154kb

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

local function setColors(string)
    local colors = separateByCharacter(string, "|")
    changeColors(colors)
end

local function drawPixel(char, pos_x, pos_y, color_foreground, color_background)
    if pos_x<0 or pos_x>160 then
        -- return
        error("unable to draw pixel: oob on x axis - cannot draw at *" .. pos_x .. "*, " ..pos_y)
    end

    if pos_y<0 or pos_y>60 then
        -- return
        error("unable to draw pixel: oob on y axis - cannot draw at " .. pos_x .. ", *" ..pos_y .. "*")
    end

    monitor.setCursorPos(pos_x+2, pos_y+3)
    monitor.blit(string.char(char), colors.toBlit(col[color_foreground+1]), colors.toBlit(col[color_background+1]))
    -- paintutils.drawPixel(pos_x+2, pos_y+3, col[_colors])
end

local function displayLine(line, pos_y) 
    local t = separateByCharacter(line, "|")                -- separate different colored chunks
    local iter = 1
    for i=1, #t do                                          -- for the amount of chunks
        local str = t[i]
        local compressed = separateByCharacter(str, "*")    -- get amount of letters
            for draw=0, tonumber(compressed[1]) do          
                local params = separateByCharacter(compressed[2], ",")
                drawPixel(params[1], iter, pos_y, tonumber(params[2]), tonumber(params[3]))
                iter = iter + 1
            end
    end
end

local function displayLines(string)
    local t = separateByCharacter(string, "-")              -- separate lines from one another
    local iter = 1
    for i=1, #t do                                          -- set up line iterator(keep track of lines)
        local params = separateByCharacter(t[i], "|")
        local pos = separateByCharacter(params[1], ",")
        local character = separateByCharacter(params[2], ",")

        drawPixel(character[1], tonumber(pos[2]), tonumber(pos[1]), tonumber(character[2]), tonumber(character[3]))
    end
end

local function setFrame(string)
    local framedata = separateByCharacter(string, "=") 
    -- print(framedata[1])                             -- frame number
    -- setColors(framedata[2])                         -- color scheme
    setColors(framedata[2])
    displayLines(framedata[3])                      -- actual frame data
end

local ws

local function connectToWS()
    ws = assert(http.websocket("127.0.0.1:3000"))
    Frametime = ws.receive()
    print("running animation @ " .. Frametime .. "fps")
    Framecount = ws.receive()
    print(Framecount)
    Framecount = tonumber(Framecount)
    print(Framecount)
    os.queueEvent("player_start")
    while true do
        local response = ws.receive()
        if response == "eof" then
            ws.close()
            print("websocket closed")
            break
        else
            local frames = separateByCharacter(response, "#")
            print("Pushing frame into framebuffer")
            for i=1, #frames do
                FrameBuffer.push(frames[i])
            end
            
            
            -- print(FrameBuffer.pop())
            -- setFrame(FrameBuffer.pop())
        end
        coroutine.yield()
    end
end

Framecount = 10
Frametime = 5

local function pushFrame()
    local frame = fs.open("/cc-streamvideo/test.ccv", "r").readAll()
    FrameBuffer.push("a")
    -- os.pullEvent("player_start") --wait for the player to start 
    local time = os.epoch("utc")
    local framec = 0
    while true do
        print("Started mainloop --------------------------------------")
        if not (FrameBuffer.first > FrameBuffer.last) and Framecount >= framec then 
            while (not (FrameBuffer.first > FrameBuffer.last)) do
                while(time+(1000/Frametime) < os.epoch("utc")) do
                    print("Frametime: "..os.epoch("utc")-time)
                    setFrame(frame)
                    time = os.epoch("utc")
                    framec = framec + 0;
                end
                sleep(0)
            end
        else
            -- coroutine.yield()
            break
        end
    end
    -- print(FrameBuffer.pop())
    print("No more frames to display")
    exit()

end

term.clear()
term.setCursorPos(1,1)
-- parallel.waitForAll(connectToWS, pushFrame)
-- connectToWS()

pushFrame()



monitor.setBackgroundColor(colors.black)