//! Mabel, Heir to Cragflame — `{1}{R}{W}` 3/3 Legendary Creature — Mouse Soldier.
//! Other Mice you control get +1/+1.
//! When Mabel enters, create Cragflame, a legendary colorless Equipment
//! artifact token with "Equipped creature gets +1/+1 and has vigilance,
//! trample, and haste" and equip {2}.
//!
//! The "Other Mice you control get +1/+1" anthem static has no primitive and
//! is GAP'd. The ETB trigger is wired and mints Cragflame as a legendary
//! colorless Equipment artifact token; the token's equipped-creature static
//! buff and its "equip {2}" activated ability are not expressible on a
//! TokenDefinition and are GAP'd (the token is created as bare bones).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mabel, Heir to Cragflame");
    let mouse = reg.interner_mut().intern("Mouse");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mouse);
    subtypes.0.insert(soldier);
    // Pre-intern the Equipment token subtype and name.
    reg.interner_mut().intern("Cragflame");
    reg.interner_mut().intern("Equipment");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "Other Mice you control get +1/+1." — anthem static, no primitive.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_make_cragflame,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_make_cragflame(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let cragflame = reg.interner().lookup("Cragflame").unwrap_or_default();
    let equipment = reg.interner().lookup("Equipment").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    // GAP: token's "Equipped creature gets +1/+1 and has vigilance, trample,
    // and haste" static and its "equip {2}" activated ability are not
    // expressible on a TokenDefinition; the token is minted as bare bones.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: cragflame,
            colors: ColorSet::colorless(),
            types: TypeLine::ARTIFACT.into(),
            subtypes,
            power: None,
            toughness: None,
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
