//! Loose in the Park — `{1}{G}` enchantment — Aura.
//! "Enchant land. When this enchantment enters, draw a card, then draft a card
//!  from this enchantment's spellbook and exile it. {3}: Enchanted land becomes
//!  a copy of the exiled card until end of turn and gains haste. It's still a
//!  land."
//!
//! Enchant land. The ETB "draw a card" is wired on the SelfEntersBattlefield
//! trigger. The spellbook draft/exile and the "{3}: enchanted land becomes a
//! copy of the exiled card" activated ability are not expressible (no
//! spellbook/draft or copy-card primitives in the demonstrated API) — GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Loose in the Park");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Permanent(
                ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
            ))
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

fn etb_draw(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // GAP: "draft a card from this enchantment's spellbook and exile it" and the
    // "{3}: enchanted land becomes a copy of the exiled card" activated ability
    // are not expressible.
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}
