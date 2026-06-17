//! Icingdeath, Frost Tyrant — `{2}{W}{W}` 4/3 Legendary Dragon with Flying
//! and Vigilance.
//! "When Icingdeath, Frost Tyrant dies, create Icingdeath, Frost Tongue, a
//!  legendary white Equipment artifact token with 'Equipped creature gets
//!  +2/+0,' 'Whenever equipped creature attacks, tap target creature
//!  defending player controls,' and equip {2}."
//!  (a white Equipment artifact token is minted; its +2/+0 static, the
//!   equipped-attacks trigger, the equip {2} ability, and the token's
//!   legendary supertype are GAP'd — TokenDefinition has no supertypes
//!   field and no expressible equip / equipped-creature hooks.)

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, PtValue, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Icingdeath, Frost Tyrant");
    let dragon = reg.interner_mut().intern("Dragon");
    let _token_name = reg.interner_mut().intern("Icingdeath, Frost Tongue");
    let _equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: dies_make_equipment,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn dies_make_equipment(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut subtypes = SubtypeSet::default();
    if let Some(eq) = reg.interner().lookup("Equipment") {
        subtypes.0.insert(eq);
    }
    let token_name = reg
        .interner()
        .lookup("Icingdeath, Frost Tongue")
        .unwrap_or_default();
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: token_name,
            colors: ColorSet::white(),
            types: TypeLine::ARTIFACT.into(),
            subtypes,
            power: None,
            toughness: None,
            keywords: vec![],
            // GAP: equip {2}, "Equipped creature gets +2/+0", and the
            // equipped-attacks tap trigger are not expressible on the token.
            abilities: vec![],
        },
    }]
}
