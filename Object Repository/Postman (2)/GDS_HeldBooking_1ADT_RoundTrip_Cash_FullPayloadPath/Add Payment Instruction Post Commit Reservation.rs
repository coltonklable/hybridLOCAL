<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Add Payment Instruction Post Commit Reservation</name>
   <tag></tag>
   <elementGuidId>889abb50-5a14-4e62-a7ce-91ab9a26194f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;Payment\&quot;: {\n    \&quot;id\&quot;: \&quot;payment_1\&quot;,\n    \&quot;Identifier\&quot;: {\n      \&quot;authority\&quot;: \&quot;Travelport\&quot;,\n      \&quot;value\&quot;: \&quot;A0656EFF-FAF4-456F-B061-0161008D6A5E\&quot;\n    },\n    \&quot;Amount\&quot;: {\n      \&quot;code\&quot;: \&quot;${currencyCode}\&quot;,\n      \&quot;minorUnit\&quot;: 2,\n      \&quot;currencySource\&quot;: \&quot;Charged\&quot;,\n      \&quot;approximateInd\&quot;: true,\n      \&quot;value\&quot;: ${TotalAmount}\n    },\n    \&quot;FormOfPaymentIdentifier\&quot;: {\n      \&quot;id\&quot;: \&quot;formOfPayment_1\&quot;,\n      \&quot;FormOfPaymentRef\&quot;: \&quot;formOfPayment_1\&quot;,\n      \&quot;Identifier\&quot;: {\n        \&quot;authority\&quot;: \&quot;Travelport\&quot;,\n        \&quot;value\&quot;: \&quot;A0656EFF-FAF4-456F-B061-0161008D6FOP\&quot;\n      }\n    },\n    \&quot;OfferIdentifier\&quot;: [\n      {\n        \&quot;id\&quot;: \&quot;offer_1\&quot;,\n        \&quot;offerRef\&quot;: \&quot;offer_1\&quot;,\n        \&quot;Identifier\&quot;: {\n          \&quot;authority\&quot;: \&quot;Travelport\&quot;,\n          \&quot;value\&quot;: \&quot;${offerIdentifierPostCommitt}\&quot;\n        }\n      }\n    ]\n  }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>E2ETrackingID</name>
      <type>Main</type>
      <value>${guid}</value>
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
      <value>${XAUTH_TRAVELPORT_ACCESSGROUP_1G}</value>
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
      <value>${Version}</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://api.apim-a.zu2.qa.travelport.io:443/uat/${Version}/air/paymentoffer/reservationworkbench/${eCacheId}/payments</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Version</defaultValue>
      <description></description>
      <id>11753d2a-f08d-439c-8d4a-42a08aee2f41</id>
      <masked>false</masked>
      <name>Version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.eCacheId</defaultValue>
      <description></description>
      <id>265dcdf2-bd54-4b6c-9e8d-985cf08049f5</id>
      <masked>false</masked>
      <name>eCacheId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guid</defaultValue>
      <description></description>
      <id>7769d749-461e-48b6-b645-fdfd97ba6213</id>
      <masked>false</masked>
      <name>guid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.XAUTH_TRAVELPORT_ACCESSGROUP_1G</defaultValue>
      <description></description>
      <id>984fbd4d-85fd-46ee-91d8-cf884bbb7a3a</id>
      <masked>false</masked>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP_1G</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.currencyCode</defaultValue>
      <description></description>
      <id>98107dea-bc84-46ad-9917-6d898f21ccd5</id>
      <masked>false</masked>
      <name>currencyCode</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.TotalAmount</defaultValue>
      <description></description>
      <id>1bdfb89a-8b35-4cdc-8aaa-5b91e1f6e0a3</id>
      <masked>false</masked>
      <name>TotalAmount</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.offerIdentifierPostCommitt</defaultValue>
      <description></description>
      <id>241c0358-60f4-42c7-8694-b7b464cb553f</id>
      <masked>false</masked>
      <name>offerIdentifierPostCommitt</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>