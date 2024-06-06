fn max_takoyaki() {
    proconio::input! {
        _target: i32,
        _max_tako: i32,
        _time: i32
    }
    let mut now_count: i32 = 0;
    let mut now_time: i32 = 0;
    let mut count: i32 = _target;
    for i in 0.._target {
        if now_count >= _target {
            break;
        }
        if _target - now_count > _max_tako {
            now_count = now_count + _max_tako;
            now_time = now_time + _time;
        }
        else {
            now_count = now_count + (_target - now_count);
            now_time = now_time + _time;
        }
    }
    println!("{}", now_time);
}