<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>bUpdate User CheckIn</name>
   <tag></tag>
   <elementGuidId>ab76aa97-d06e-41c8-8cd3-382d2c077b83</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;checkInId\&quot;:\&quot;checkinID\&quot;,\n  \&quot;checkedOutTime\&quot;:\&quot;checkedOutTime\&quot;,\n  \&quot;checkedOutLatitude\&quot; :12.9898,\n  \&quot;checkedOutLongitude\&quot;:12.989898,\n  \&quot;checkOutLocality\&quot;:\&quot;${checkoutLocality}\&quot;,\n}&quot;,
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
   <restUrl>http://10.130.1.49:2022/v1/api/update-user-checkin?uniqueToken=${uniqueToken}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.uniquetoken</defaultValue>
      <description></description>
      <id>596d3ee5-4fe7-4fde-bf7e-de9af33ded9d</id>
      <masked>false</masked>
      <name>uniqueToken</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.userid</defaultValue>
      <description></description>
      <id>3fa7afe8-8ed2-4938-a387-87b6ff23dd52</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>bd821e99-7f01-436f-b3ef-4eb0db5e5bc2</id>
      <masked>false</masked>
      <name>checkinID</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>024d0207-86d0-4769-9947-0f29233637eb</id>
      <masked>false</masked>
      <name>checkedOutTime</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>0dc2227f-e9b9-4350-8527-7cb56f761e85</id>
      <masked>false</masked>
      <name>checkoutLocality</name>
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

WS.verifyResponseStatusCode(response, 204)

assertThat(response.getStatusCode()).isEqualTo(204)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
