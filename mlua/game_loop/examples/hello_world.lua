local ticks
local total_time

function set_up()
    ticks = 0
    total_time = 0
end

function update(dt)
    total_time = total_time + dt
    if ticks < 100 then
        ticks = ticks + 1
        print(total_time)
    else
        stop()
    end
end