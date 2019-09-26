<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>nHoliday Master</name>
   <tag></tag>
   <elementGuidId>c1248293-cc62-4152-bc08-1de376cb4a19</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;versionTableId\&quot; :\&quot;${versiontableId}\&quot;,\n\t\&quot;syncType\&quot;:\&quot;F\&quot;,\n\t\&quot;MaxUserVersion\&quot;:\&quot;1\&quot;\n}\n\n&quot;,
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
   <restUrl>http://10.130.1.49:2022/v1/api/holiday-master?uniqueToken=${uniqueToken}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>6b051272-c91a-4f8e-900c-298acf4f5d58</id>
      <masked>false</masked>
      <name>versiontableId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.uniquetoken</defaultValue>
      <description></description>
      <id>07929ee0-2670-4e0d-8947-515748c0a55b</id>
      <masked>false</masked>
      <name>uniqueToken</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.userid</defaultValue>
      <description></description>
      <id>992fec42-7ff0-441e-94ec-5f42253f8b2f</id>
      <masked>false</masked>
      <name>userId</name>
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


WS.verifyElementPropertyValue(response, 'holidaysinfo[0].holidayId', &quot;86858bb1-4d53-49cf-bccc-0477e5cb0a8f&quot;)


WS.verifyElementPropertyValue(response, 'holidaysinfo[19].holidayId', &quot;e6aed40f-28bf-4dfe-9fae-fbe32678c0c8&quot;)


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
