<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>z1Mobile Tracking---ND</name>
   <tag></tag>
   <elementGuidId>9c127438-4d34-42b8-bbe5-3e319dc7c6f7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;mobiletrackingInfo\&quot;: [{\n\t\t\&quot;createdDate\&quot;:\&quot;${createdDatE}\&quot;,\n\t\t\&quot;ctid\&quot;:\&quot;${ctiD}\&quot;,\n\t\t\&quot;submittedDate\&quot;:\&quot;${submittedDatE}\&quot;,\n\t\t\&quot;trackingGoogleAddress\&quot;: \&quot;Automation on Kataloan studio \&quot;,\n\t\t\&quot;trackingLatitude\&quot;: \&quot;37.4219983\&quot;,\n\t\t\&quot;trackingLongitude\&quot;: \&quot;-122.084\&quot;\n\t}]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tenantId</name>
      <type>Main</type>
      <value>b68f64ef-0f8f-47a9-b192-ad8e9e6a5d20</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>userId</name>
      <type>Main</type>
      <value>${userId}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://10.130.1.49:2022/v1/api/mobile-tracking?uniqueToken=${uniqueToken}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.uniquetoken</defaultValue>
      <description></description>
      <id>54e95a0f-aa2f-47e2-8058-05cbd892b1ad</id>
      <masked>false</masked>
      <name>uniqueToken</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.userid</defaultValue>
      <description></description>
      <id>a4d7f36c-7389-456a-9ce1-4703357d46be</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3bdbb990-a3ac-43c5-ac80-7b955c89d1b8</id>
      <masked>false</masked>
      <name>createdDatE</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>377db81e-bf02-4256-95e6-0a7b4e4ddf11</id>
      <masked>false</masked>
      <name>ctiD</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>92b28d61-3139-4763-8026-7677c834595a</id>
      <masked>false</masked>
      <name>submittedDatE</name>
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
