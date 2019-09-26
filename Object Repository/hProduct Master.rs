<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>hProduct Master</name>
   <tag></tag>
   <elementGuidId>3ab54031-da88-4e65-8634-59b49476b242</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;versionTableId\&quot; :\&quot;${versiontableId}\&quot;,\n  \&quot;syncType\&quot; : \&quot;f\&quot;\n   \n  \n}&quot;,
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
   <restUrl>http://10.130.1.49:2022/v1/api/product-master?uniqueToken=${uniqueToken}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d3ef6bf2-1ff9-4b13-9b55-aae9d584748b</id>
      <masked>false</masked>
      <name>versiontableId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.userid</defaultValue>
      <description></description>
      <id>f4cd8fdc-4291-4517-9aa1-bdd3123faa95</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.uniquetoken</defaultValue>
      <description></description>
      <id>4b7b139a-a04e-4a2f-8df1-9635ea2a70d4</id>
      <masked>false</masked>
      <name>uniqueToken</name>
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

assertThat(response.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(response, 'poductInfo[0].productId', &quot;584b0656-d2de-4c88-ab96-03b012966248&quot;)
WS.verifyElementPropertyValue(response, 'poductInfo[0].productName', &quot;TOPSHOT 60OD HERB&quot;)


WS.verifyElementPropertyValue(response, 'poductInfo[43].productId', &quot;04211bcd-fb61-4146-99c4-f881ee3f11ea&quot;)
WS.verifyElementPropertyValue(response, 'poductInfo[43].productName', &quot;BEAM 75WP&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
