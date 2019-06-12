echo "Test-Case 1: Cow-User Logup: "
curl http://localhost:6789/cow_logup\
 -H "Content-Type:application/json"\
 -d '{"username":"SDCS-Yard", "userid":"wechat12306", "wechat_ok":true, "logup_mode":false, "email":"1377278218@qq.com", "phone":"13432769342", "infos":"A New Comer", "organization":"SYSU-SDCS"}'
echo " "

echo "Test-Case 2: Student-User Logup: "
curl http://localhost:6789/student_logup\
 -H "Content-Type:application/json"\
 -d '{"username":"XiaoMIng", "userid":"wechat12307", "wechat_ok":true, "logup_mode":true, "email":"1377278216@qq.com", "phone":"13432769341", "infos":"A student", "school_name":"中山大学", "student_id":"16340001", "major":"CS", "year":3}'


# 注意事项
# 1. 必须通过POST方式发送请求，Content-Type必须为application/json
# 2. 用户的昵称允许重复，但是一个微信只能注册一个帐号，微信ID，邮箱，手机号，学号不允许重复
