//! Monologue Tax — `{2}{W}` enchantment (Streets of New Capenna Commander,
//! 2022). "Whenever an opponent casts their second spell each turn, you
//! create a Treasure token."
//!
//! Opponent `SpellCast` trigger gated by an intervening-if reading
//! `script::spells_cast_this_turn` over an opponent-controlled filter — the
//! firing cast is already logged, so the gate is exact on the second spell
//! (per-opponent counting in multiplayer is a documented approximation:
//! the count aggregates ALL opponents' spells, exact in two-player games).

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Monologue Tax");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: Some(if_second_opponent_spell),
                effect: tax_treasure,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "their second spell each turn" — exact on the second opponent spell of
/// the turn (the firing cast is already in the turn's event log). GAP:
/// counts all opponents' spells together rather than per-opponent
/// (equivalent in two-player games).
fn if_second_opponent_spell(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    script::spells_cast_this_turn(
        s,
        &ObjectFilter::new().controlled_by(ControllerConstraint::Opponent),
        you,
    ) == 2
}

/// "…you create a Treasure token."
fn tax_treasure(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Treasure,
        count: 1,
    }]
}
