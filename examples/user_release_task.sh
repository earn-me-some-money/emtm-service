echo "Test-Case 1: Cow-User Release Task: "
curl http://localhost:8080/release_task\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12306", "release_mode":false, "task_name":"Find Fantasy", "task_intro":"Test", "task_mode":0, "task_request":{"grade":1, "major":"CS", "task_exper":10, "credit_score":90, "max_participants":50}, "task_pay":10, "task_risk":5, "task_time_limit":"2019-05-26:22-00"}'
echo ""

echo "Test-Case 2: Student-User Release Task: "
curl http://localhost:8080/release_task\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12307", "release_mode":true, "task_name":"Pick Me Up", "task_intro":"Test", "task_mode":1, "task_request":{"grade":-1, "major":"", "task_exper":-1, "credit_score":-1, "max_participants":-1}, "task_pay":10, "task_risk":5, "task_time_limit":"2019-05-26:22-00"}'
echo ""


# 注意事项
# 1. 学生发布任务时，也需要将task_request字段传输过来，由于Rust后台静态分析的原因
# 可以使用一系列默认值填充不需要填写的字段，不会进行读取