<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>ReceiptResource</name>
   <tag></tag>
   <elementGuidId>b54e6c0e-8293-43c2-9048-4dbd5eed6658</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
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
      <name>E2ETrackingID</name>
      <type>Main</type>
      <value>${guid}</value>
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
      <name>Content-Version</name>
      <type>Main</type>
      <value>${Version}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept-Version</name>
      <type>Main</type>
      <value>${Version}</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://api.apim-a.zu2.qa.travelport.io:443/uat/${Version}/air/receipt/reservations/${PNR}/receipts?ReceiptType=CONFIRMATIONTICKET</restUrl>
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
      <id>1bf1affb-4977-4701-b8f4-1f9514fd043e</id>
      <masked>false</masked>
      <name>Version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.PNR</defaultValue>
      <description></description>
      <id>f41859f8-956a-4f95-95af-ce94b258e71c</id>
      <masked>false</masked>
      <name>PNR</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guid</defaultValue>
      <description></description>
      <id>1cc31ba4-b934-42a1-ab3c-619d44195989</id>
      <masked>false</masked>
      <name>guid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.XAUTH_TRAVELPORT_ACCESSGROUP_1G</defaultValue>
      <description></description>
      <id>8c4fe972-831c-4d64-a466-3960e6da1381</id>
      <masked>false</masked>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP_1G</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>