//! Chancellor of the Forge — `{4}{R}{R}{R}` 5/5 Phyrexian Giant.
//!
//! "You may reveal this card from your opening hand. If you do, at the
//! beginning of the first upkeep, create a 1/1 red Phyrexian Goblin
//! creature token with haste." + "When this creature enters, create X
//! 1/1 red Phyrexian Goblin creature tokens with haste, where X is the
//! number of creatures you control."
//!
//! Decomposition:
//! * Opening-hand reveal ability → no opening-hand mechanic in the
//!   catalog. GAP (omitted ability).
//! * ETB trigger → create X Goblin tokens, X computed at resolution
//!   via `script::count_matching` over the creatures you control; one
//!   `Effect::CreateToken` per X.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chancellor of the Forge");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let giant = reg.interner_mut().intern("Giant");
    let _goblin = reg.interner_mut().intern("Goblin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(giant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // GAP: "You may reveal this card from your opening hand. If you do, at the
    // beginning of the first upkeep, create a 1/1 red Phyrexian Goblin token with
    // haste." — no opening-hand reveal mechanic in the catalog.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_make_goblins,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// ETB: create X 1/1 red Phyrexian Goblin tokens with haste, where X is
/// the number of creatures you control.
fn etb_make_goblins(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let x = script::count_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let phyrexian = reg.interner().lookup("Phyrexian").unwrap_or_default();
    let goblin = reg.interner().lookup("Goblin").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(goblin);
    let token = TokenDefinition {
        name: goblin,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Haste],
        abilities: vec![],
    };
    (0..x)
        .map(|_| Effect::CreateToken {
            controller: trig.controller,
            token: token.clone(),
        })
        .collect()
}
