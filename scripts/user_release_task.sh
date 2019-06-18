echo "Test-Case 1: Cow-User Release Task: "
curl http://localhost:6789/release_task\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12306", "task_name":"Find Fantasy", "task_intro":"Test", "task_mode":0, "task_request":{"min_grade":1, "max_grade":3, "major":"CS", "task_exper":10, "credit_score":90, "max_participants":50}, "task_pay":10, "task_risk":5, "task_time_limit":"2019-06-27:22-00"}'
echo ""

echo "Test-Case 2: Student-User Release Task: "
curl http://localhost:6789/release_task\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12307", "task_name":"Pick Me Up3", "task_intro":"Test", "task_mode":1, "task_request":{"max_participants":5}, "task_pay":-10, "task_risk":5, "task_time_limit":"2019-06-27:22-00"}'
echo ""

