//! Eon Frolicker — `{2}{U}{U}` 5/5 Elemental Otter.
//! Flying.
//! When this creature enters, if you cast it, target opponent takes an
//! extra turn after this one. Until your next turn, you and planeswalkers
//! you control gain protection from that player.
//!
//! Flying is a base keyword. The ETB gives the targeted opponent an
//! extra turn. The "if you cast it" gate (no cast-condition predicate)
//! and the "until your next turn, protection from that player" clause
//! (no per-player protection primitive) are GAPs.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eon Frolicker");
    let elemental = reg.interner_mut().intern("Elemental");
    let otter = reg.interner_mut().intern("Otter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(otter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP (intervening-if): "if you cast it" — no cast-condition
                // predicate is available; left ungated.
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: opponent_extra_turn,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            }),
    )
}

fn opponent_extra_turn(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    // GAP: "Until your next turn, you and planeswalkers you control gain
    // protection from that player." No per-player protection primitive.
    vec![Effect::ExtraTurn { player: *p }]
}
