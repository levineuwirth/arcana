//! Rune of Sustenance — `{1}{W}` enchantment — Aura Rune.
//! "Enchant permanent. When this Aura enters, draw a card. As long as
//!  enchanted permanent is a creature, it has lifelink. As long as enchanted
//!  permanent is an Equipment, it has \"Equipped creature has lifelink.\""
//!
//! ETB draws a card. The two conditional ("as long as …") grants depend on
//! the host's type, which the demonstrated `attached_*` builders can't gate
//! on — GAP'd. `with_enchant(Permanent)` carries the enchant target.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rune of Sustenance");
    let aura = reg.interner_mut().intern("Aura");
    let rune = reg.interner_mut().intern("Rune");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    subtypes.0.insert(rune);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Permanent(ObjectFilter::permanent()))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: type-conditional ("as long as enchanted permanent is a
    // creature/Equipment") grants of lifelink are not expressible via the
    // demonstrated attached_* builders (no host-type gating).
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}
