<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>y1Mobile App Version</name>
   <tag></tag>
   <elementGuidId>35ed0559-d2ee-4c19-a603-22ba7b655471</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;os\&quot;:\&quot;${oparatingSystem}\&quot;,\n\t\&quot;versionNumber\&quot;:\&quot;${versionNumber}\&quot;\n}\n\n&quot;,
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
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://10.130.1.49:2022/v1/api/mobile-app-version?uniqueToken=${uniqueToken}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.uniquetoken</defaultValue>
      <description></description>
      <id>cc498d0e-267c-48c1-ae92-42430a8d26b8</id>
      <masked>false</masked>
      <name>uniqueToken</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>187a4dbe-b4ca-4230-a54c-58d7164efebf</id>
      <masked>false</masked>
      <name>oparatingSystem</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3db30068-8aff-4c26-b03e-aafceaefe68d</id>
      <masked>false</masked>
      <name>versionNumber</name>
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


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
