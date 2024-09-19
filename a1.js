let now = new Date();
// let tasks = global.get("all_for_timer_task")
let tasks = [{ "id": 49, "start_time": "01:00:00", "end_time": "18:00:00", "on_duration": 5, "on_unit": 0, "off_unit": 0, "off_duration": 5, "times": 0, "ope": ["1"] }];
if (tasks == undefined) {
    tasks = []
}
let opeList = [];

let task = tasks[0];
    let run_task = false;
    if ((task.start_time.split(":")).length == 2) {                    //
        task.start_time = task.start_time + ":00"      //
    }
    if ((task.end_time.split(":")).length == 2) {                    //
        task.end_time = task.end_time + ":00"      //
    }
    let startArr = task.start_time.split(":");
    let endArr = task.end_time.split(":");
    let start_time = new Date();
    start_time.setHours(startArr[0]);
    start_time.setMinutes(startArr[1]);
    start_time.setSeconds(startArr[2]);                                         //
    let today_end_time = new Date();
    today_end_time.setHours(endArr[0]);
    today_end_time.setMinutes(endArr[1]);
    today_end_time.setSeconds(endArr[2]);                                          //
    //判断是否跨天及当前是否属于循环时间段
    if (today_end_time < start_time) {
        let tomorrow_end_time = new Date();
        if ((tomorrow_end_time.split(":")).length == 2) {                    //
            tomorrow_end_time = tomorrow_end_time + ":00"      //
        }
        tomorrow_end_time.setHours(endArr[0]);
        tomorrow_end_time.setMinutes(endArr[1]);
        tomorrow_end_time.setSeconds(endArr[2]);                                            //
        tomorrow_end_time.setDate(today_end_time.getDate() + 1);
        if (today_end_time > now || (start_time <= now && tomorrow_end_time > now)) {
            run_task = true;
        }
    } else if (task.start_time == task.end_time) {
        run_task = true;
    }
    if (start_time <= now && today_end_time > now) {
        run_task = true;
    }


    if (run_task) {
        if (typeof task.status === 'undefined') {
            task.status = 1;
            task.sustain_duration = 0;
            task.ope.forEach(function (ele) {
                opeList.push({
                    id: task.id,
                    state: 1,
                    key: ele
                });
            });

        } else {
            task.sustain_duration += 1;
            if (task.off_unit == 0) {// secend
                if (task.status == 0 && task.sustain_duration >= task.off_duration) {
                    task.status = 1
                    task.sustain_duration = 0;
                    task.ope.forEach(function (ele) {
                        opeList.push({
                            id: task.id,
                            state: 1,
                            key: ele
                        });
                    });
                }
            }
            if (task.off_unit == 1) { // mintes
                if (task.status == 0 && task.sustain_duration >= (task.off_duration * 60)) {          //
                    task.status = 1
                    task.sustain_duration = 0;
                    task.ope.forEach(function (ele) {
                        opeList.push({
                            id: task.id,
                            state: 1,
                            key: ele
                        });
                    });
                }
            }
            if (task.off_unit == 2) {// hours
                if (task.status == 0 && task.sustain_duration >= (task.off_duration * 60 * 60)) {       //
                    task.status = 1
                    task.sustain_duration = 0;
                    task.ope.forEach(function (ele) {
                        opeList.push({
                            id: task.id,
                            state: 1,
                            key: ele
                        });
                    });
                }
            }
            if (task.on_unit == 0) {              //
                if (task.status == 1 && task.sustain_duration >= task.on_duration) {
                    task.status = 0;
                    task.sustain_duration = 0;
                    task.ope.forEach(function (ele) {
                        opeList.push({
                            id: task.id,
                            state: 2,
                            key: ele
                        });
                    });
                }
            }
            if (task.on_unit == 1) {              //
                if (task.status == 1 && task.sustain_duration >= task.on_duration * 60) {            //
                    task.status = 0;
                    task.sustain_duration = 0;
                    task.ope.forEach(function (ele) {
                        opeList.push({
                            id: task.id,
                            state: 2,
                            key: ele
                        });
                    });
                }
            }
            if (task.on_unit == 2) {              //
                if (task.status == 1 && task.sustain_duration >= task.on_duration * 60 * 60) {
                    task.status = 0;
                    task.sustain_duration = 0;
                    task.ope.forEach(function (ele) {
                        opeList.push({
                            id: task.id,
                            state: 2,
                            key: ele
                        });
                    });
                }
            }
        }
    } else {
        //不在循环时间段但上次循环最后状态是开启，直接执行关闭命令
        if (task.status == 1) {
            delete task.status;
            delete task.sustain_duration;
            task.ope.forEach(function (ele) {
                opeList.push({
                    id: task.id,
                    state: 2,
                    key: ele
                });
            });
        }
    }

msg.opeList = opeList;
return msg;