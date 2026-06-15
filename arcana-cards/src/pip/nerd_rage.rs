//! Nerd Rage — `{2}{U}` enchantment — Aura.
//! "Enchant creature. When this Aura enters, draw two cards. Enchanted
//!  creature has \"You have no maximum hand size\" and \"Whenever this
//!  creature attacks, if you have ten or more cards in hand, it gets
//!  +10/+10 until end of turn.\""
//!
//! The ETB "draw two cards" is fully expressible. The granted "no maximum
//! hand size" static has no attached continuous-effect builder, and the
//! granted host-attack conditional pump is gated on an intervening-if
//! (ten-or-more cards in hand) that `AttachedCreatureDoes` cannot carry —
//! both GAP rather than fire unconditionally.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nerd Rage");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_draw_two,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::AttachedCreatureDoes {
                    condition: Box::new(TriggerCondition::SelfAttacks),
                },
                intervening_if: None,
                effect: on_host_attacks,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_draw_two(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: granted "You have no maximum hand size" static has no attached builder.
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 2,
    }]
}

fn on_host_attacks(
    _state: &GameState,
    _trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: granted host-attack +10/+10 is gated by an intervening-if
    // ("if you have ten or more cards in hand") that AttachedCreatureDoes
    // cannot carry; would fire unconditionally otherwise.
    Vec::new()
}
