//! The Keeper of Four Scythes — `{3}{W}{W}` 1/1 Legendary Human Wizard.
//!
//! Echo {3}{W}{W} (GAP — Echo is not in the usable keyword surface.)
//! When this creature enters, create four Scythe tokens. (They're
//!   Equipment with "Equipped creature gets +1/+1" and equip {2}.)
//! Equipped creatures you control get +1/+1. (static — GAP.)
//!
//! The ETB creating four Scythe Equipment artifact tokens is expressible
//! as four bare `CreateToken` effects; the tokens' internal equip {2} and
//! "Equipped creature gets +1/+1" static are GAP'd (token-side equip /
//! attach-buff abilities aren't expressible via `TokenDefinition` here).
//! The Echo upkeep mechanic and the controller-wide "Equipped creatures
//! you control get +1/+1" static are also GAP'd.

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
    let name = reg.interner_mut().intern("The Keeper of Four Scythes");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let _scythe = reg.interner_mut().intern("Scythe");
    let _equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    // GAP: Echo {3}{W}{W} — not in the usable keyword surface.
    // GAP: "Equipped creatures you control get +1/+1" — static.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_make_scythes,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_make_scythes(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let scythe = reg.interner().lookup("Scythe").unwrap_or_default();
    let equipment = reg.interner().lookup("Equipment").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    // GAP: the tokens' equip {2} and "Equipped creature gets +1/+1"
    // static are omitted (token-side equip / attach-buff not expressible).
    let token = TokenDefinition {
        name: scythe,
        colors: ColorSet::colorless(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: trig.controller, token: token.clone() },
        Effect::CreateToken { controller: trig.controller, token: token.clone() },
        Effect::CreateToken { controller: trig.controller, token: token.clone() },
        Effect::CreateToken { controller: trig.controller, token },
    ]
}
