<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Add FOP Post Commit Reservation - Credit UA AX</name>
   <tag></tag>
   <elementGuidId>befb2fb3-2cd0-44b7-ac6f-46ea93209d35</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;FormOfPaymentPaymentCard\&quot;: {\r\n        \&quot;id\&quot;: \&quot;formOfPayment_5\&quot;,\r\n        \&quot;FormOfPaymentRef\&quot;: \&quot;formOfPayment_5\&quot;,\r\n        \&quot;Identifier\&quot;: {\r\n            \&quot;authority\&quot;: \&quot;Travelport\&quot;,\r\n            \&quot;value\&quot;: \&quot;A0656EFF-FAF4-456F-B061-0161008D6FOP\&quot;\r\n        },\r\n        \&quot;PaymentCard\&quot;: {\r\n            \&quot;@type\&quot;: \&quot;PaymentCardDetail\&quot;,\r\n            \&quot;id\&quot;: \&quot;paymentCard_4\&quot;,\r\n            \&quot;expireDate\&quot;: \&quot;0524\&quot;,\r\n            \&quot;CardType\&quot;: \&quot;Credit\&quot;,\r\n            \&quot;CardCode\&quot;: \&quot;AX\&quot;,\r\n            \&quot;CardHolderName\&quot;: \&quot;JANE DOE\&quot;,\r\n            \&quot;CardNumber\&quot;: {\r\n                \&quot;@type\&quot;: \&quot;CardNumber\&quot;,\r\n                \&quot;PlainText\&quot;: \&quot;345678000000007\&quot;\r\n            },\r\n            \&quot;SeriesCode\&quot;: {\r\n                \&quot;PlainText\&quot;: \&quot;4111\&quot;\r\n            }\r\n           \r\n        }\r\n    }\r\n} &quot;,
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
   <restUrl>${ts_environment}/${version}/air/payment/reservationworkbench/${eCacheIdUa}/formofpayment</restUrl>
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
      <id>5bbec96a-0e73-4166-b76b-6b5f700caceb</id>
      <masked>false</masked>
      <name>ts_environment</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.version</defaultValue>
      <description></description>
      <id>52d9c48e-af86-478e-ba0a-cecd007a1e49</id>
      <masked>false</masked>
      <name>version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.eCacheIdUa</defaultValue>
      <description></description>
      <id>230da28f-974d-4605-8bc4-9c23558f1e96</id>
      <masked>false</masked>
      <name>eCacheIdUa</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>327af1f9-3db9-4dfb-be88-a9b63ebd32b7</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guid</defaultValue>
      <description></description>
      <id>c3a040fb-7639-4639-bd1f-c799236acd9b</id>
      <masked>false</masked>
      <name>guid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7</defaultValue>
      <description></description>
      <id>1c0f50c7-8760-4848-9964-cf21f2b17ca5</id>
      <masked>false</masked>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
