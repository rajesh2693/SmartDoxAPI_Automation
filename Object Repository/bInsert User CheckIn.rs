<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>bInsert User CheckIn</name>
   <tag></tag>
   <elementGuidId>687e34f3-ecad-49a0-8de0-d0dbfad6a1f7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;checkInId\&quot;:\&quot;${checkInID}\&quot;,\n  \&quot;checkedInTime\&quot;:\&quot;${checkInTime}\&quot;,\n  \&quot;checkInLocality\&quot;:\&quot;${checkinLocality}\&quot;,\n  \&quot;checkedInLatitude\&quot; :12.89898,\n  \&quot;checkedInLongitude\&quot;:12.089898,\n}\n\n\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tenantId</name>
      <type>Main</type>
      <value>b68f64ef-0f8f-47a9-b192-ad8e9e6a5d20</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>userId</name>
      <type>Main</type>
      <value>${userId}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://10.130.1.49:2022/v1/api/insert-user-checkin?uniqueToken=${uniqueToken}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.uniquetoken</defaultValue>
      <description></description>
      <id>42f2896c-594c-4940-beac-44b7cd773cc5</id>
      <masked>false</masked>
      <name>uniqueToken</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.uniquetoken</defaultValue>
      <description></description>
      <id>ca360d78-35d4-4e4d-a9ec-91e02dc476b5</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>a7d228f1-daef-4b88-abb7-dcdc135050da</id>
      <masked>false</masked>
      <name>checkInTime</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f731ab6a-b259-4385-b31c-a8233ce7bbd6</id>
      <masked>false</masked>
      <name>checkInID</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d70bded5-f16e-46c0-a213-db9cc4759597</id>
      <masked>false</masked>
      <name>checkinLocality</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
