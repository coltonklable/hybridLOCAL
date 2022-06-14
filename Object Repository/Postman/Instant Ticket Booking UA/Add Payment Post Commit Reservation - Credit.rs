<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Add Payment Post Commit Reservation - Credit</name>
   <tag></tag>
   <elementGuidId>a2aa8980-a7b5-4ba8-8c05-a80250a7f5db</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;Payment\&quot;: {\n    \&quot;id\&quot;: \&quot;paymentCard_4\&quot;,\n    \&quot;Identifier\&quot;: {\n      \&quot;authority\&quot;: \&quot;Travelport\&quot;,\n      \&quot;value\&quot;: \&quot;A0656EFF-FAF4-456F-B061-0161008D6A6E\&quot;\n    },\n    \&quot;Amount\&quot;: {\n      \&quot;code\&quot;: \&quot;${currencyCode}\&quot;,  \n      \&quot;minorUnit\&quot;: 2, \n      \&quot;currencySource\&quot;: \&quot;Charged\&quot;, \n      \&quot;approximateInd\&quot;: true, \n      \&quot;value\&quot;: \&quot;${total_price_amount}\&quot;\n    },\n    \&quot;FormOfPaymentIdentifier\&quot;: {\n      \&quot;id\&quot;: \&quot;formOfPayment_5\&quot;,\n      \&quot;FormOfPaymentRef\&quot;: \&quot;formOfPayment_5\&quot;,\n      \&quot;Identifier\&quot;: {\n          \&quot;authority\&quot;: \&quot;Travelport\&quot;,\n          \&quot;value\&quot;: \&quot;A0656EFF-FAF4-456F-B061-0161008D6FOP\&quot;\n      }\n      \n    },\n    \&quot;OfferIdentifier\&quot;: [\n      {\n        \&quot;id\&quot;: \&quot;o0\&quot;,\n        \&quot;offerRef\&quot;: \&quot;o0\&quot;,\n        \&quot;Identifier\&quot;: {\n          \&quot;authority\&quot;: \&quot;${price_authority}\&quot;,\n          \&quot;value\&quot;: \&quot;${price_identfier_value}\&quot;\n        }\n      }\n    ]\n  }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
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
      <name>Authorization</name>
      <type>Main</type>
      <value>${token}</value>
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
      <name>XAUTH_TRAVELPORT_ACCESSGROUP</name>
      <type>Main</type>
      <value>${XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7}</value>
   </httpHeaderProperties>
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
      <name>Content-Version</name>
      <type>Main</type>
      <value>${version}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept-Version</name>
      <type>Main</type>
      <value>6</value>
   </httpHeaderProperties>
   <katalonVersion>8.0.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${ts_environment}/${version}/air/paymentoffer/reservationworkbench/${reservationIdUa}/payments</restUrl>
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
      <id>a52b9c4c-fd57-4076-be91-7b552ab58c31</id>
      <masked>false</masked>
      <name>ts_environment</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.version</defaultValue>
      <description></description>
      <id>f5ddfdde-26d1-41b3-a385-9ba57da407fb</id>
      <masked>false</masked>
      <name>version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.reservationIdUa</defaultValue>
      <description></description>
      <id>a5b83c3e-71be-4993-85ed-28d18b96ee05</id>
      <masked>false</masked>
      <name>reservationIdUa</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guid</defaultValue>
      <description></description>
      <id>82ba5462-a211-4f26-9257-0f22f214cfed</id>
      <masked>false</masked>
      <name>guid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>6d93b89a-920b-465f-b188-9b5961ad6c80</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7</defaultValue>
      <description></description>
      <id>80796c62-2e8d-4b67-ace7-b6f8f65402a6</id>
      <masked>false</masked>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.currencyCode_held</defaultValue>
      <description></description>
      <id>b5b46d9f-27fd-4a8e-a554-0a3ba0287af4</id>
      <masked>false</masked>
      <name>currencyCode</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.total_price_amount_held</defaultValue>
      <description></description>
      <id>aea286f6-d1c3-428c-bd35-4dcf8c05f72a</id>
      <masked>false</masked>
      <name>total_price_amount</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.price_authority</defaultValue>
      <description></description>
      <id>bfeffc27-17be-47d1-87d2-dc1d5074348a</id>
      <masked>false</masked>
      <name>price_authority</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.price_authority_held</defaultValue>
      <description></description>
      <id>fd6194fd-ee4f-466e-a7e1-1f0706eca61d</id>
      <masked>false</masked>
      <name>price_identfier_value</name>
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
