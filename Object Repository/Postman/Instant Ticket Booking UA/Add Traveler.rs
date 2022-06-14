<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Add Traveler</name>
   <tag></tag>
   <elementGuidId>2e110120-90ab-4798-8a56-59fd26784b3b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\&quot;Traveler\&quot;:{\&quot;dob\&quot;:\&quot;1984-02-22\&quot;,\&quot;gender\&quot;:\&quot;Male\&quot;,\&quot;passengerTypeCode\&quot;:\&quot;ADT\&quot;,\&quot;PersonName\&quot;:{\&quot;@type\&quot;:\&quot;PersonNameDetail\&quot;,\&quot;Prefix\&quot;:\&quot;Mr\&quot;,\&quot;Given\&quot;:\&quot;JAMES\&quot;,\&quot;Surname\&quot;:\&quot;HARKNESS\&quot;},\&quot;Telephone\&quot;:[{\&quot;@type\&quot;:\&quot;Telephone\&quot;,\&quot;countryAccessCode\&quot;:\&quot;1\&quot;,\&quot;areaCityCode\&quot;:\&quot;909\&quot;,\&quot;phoneNumber\&quot;:\&quot;212456121\&quot;,\&quot;extension\&quot;:\&quot;1243\&quot;,\&quot;id\&quot;:\&quot;4\&quot;,\&quot;cityCode\&quot;:\&quot;ORD\&quot;,\&quot;role\&quot;:\&quot;Office\&quot;},{\&quot;@type\&quot;:\&quot;Telephone\&quot;,\&quot;countryAccessCode\&quot;:\&quot;1\&quot;,\&quot;areaCityCode\&quot;:\&quot;303\&quot;,\&quot;phoneNumber\&quot;:\&quot;111456121\&quot;,\&quot;extension\&quot;:\&quot;1243\&quot;,\&quot;id\&quot;:\&quot;4\&quot;,\&quot;cityCode\&quot;:\&quot;ORD\&quot;,\&quot;role\&quot;:\&quot;Office\&quot;}],\&quot;Email\&quot;:[{\&quot;comment\&quot;:\&quot;Primary Email Id\&quot;,\&quot;value\&quot;:\&quot;TravelerOne@gmail.com\&quot;}]}}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
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
      <name>Authorization</name>
      <type>Main</type>
      <value>${token}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>trackingId</name>
      <type>Main</type>
      <value>${guid}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP</name>
      <type>Main</type>
      <value>${XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Version</name>
      <type>Main</type>
      <value>${version}</value>
   </httpHeaderProperties>
   <katalonVersion>8.0.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${ts_environment}/${version}/air/book/traveler/reservationworkbench/${reservationIdUa}/travelers</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.ts_environment</defaultValue>
      <description></description>
      <id>213d0a22-391c-4aa8-a264-db7929b89788</id>
      <masked>false</masked>
      <name>ts_environment</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.version</defaultValue>
      <description></description>
      <id>e4f1a11b-9ef0-40b3-87e1-250169c9d6b8</id>
      <masked>false</masked>
      <name>version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.reservationIdUa</defaultValue>
      <description></description>
      <id>9f0b7756-9433-4693-81cc-e4835ffcbafe</id>
      <masked>false</masked>
      <name>reservationIdUa</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>9af06875-3808-496c-90d4-44d9de7c0d0c</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guid</defaultValue>
      <description></description>
      <id>5fa5582b-55e3-4838-9150-b833bf26d326</id>
      <masked>false</masked>
      <name>guid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7</defaultValue>
      <description></description>
      <id>659a5e8e-f80a-44f5-ad57-b9a8423d7eab</id>
      <masked>false</masked>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
