//! Voracious Dragon — `{3}{R}{R}` 4/4 Dragon with Flying and Devour 1.
//!
//! Oracle:
//! * Flying (keyword).
//! * Devour 1 (keyword — the engine handles the sacrifice-and-grow enters-with).
//! * When this creature enters, it deals damage to any target equal to twice
//!   the number of Goblins it devoured.
//!
//! The ETB damage count ("twice the number of Goblins it devoured") needs the
//! number of Goblins sacrificed to Devour, for which there is no
//! `PendingTrigger` accessor — so the dynamic-count damage effect is GAP'd
//! (cf. Tar Fiend). The any-target requirement is still declared on the
//! trigger for catalog fidelity; the effect returns no damage.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Voracious Dragon");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Devour(1)],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_devour_damage,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::any_target()],
        }),
    )
}

fn etb_devour_damage(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: damage "equal to twice the number of Goblins it devoured" — no
    // PendingTrigger accessor for the number of (Goblin) creatures sacrificed
    // to Devour, so the dynamic damage count is not computable.
    Vec::new()
}
