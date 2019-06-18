echo "Edit Student Info Test Case 1: (False)"
curl http://localhost:6789/edit_stu_info\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12306", "new_email":"1377278212@qq.com", "new_phone":"13432769345", "new_infos":"幻想乡~", "new_major":"APP", "new_year":2}'
echo ""

echo "Edit Student Info Test Case 2: (True)"
curl http://localhost:6789/edit_stu_info\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12307", "new_email":"1377278216@qq.com", "new_phone":"13432769344", "new_infos":"中文~", "new_major":"APP", "new_year":2}'
echo ""

echo "Edit Student Info Test Case 3: (True)"
curl http://localhost:6789/edit_stu_info\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12308", "new_email":"1377278216@qq.com", "new_phone":"13432769341", "new_infos":"幻想乡~", "new_major":"APP", "new_year":2}'
echo ""
