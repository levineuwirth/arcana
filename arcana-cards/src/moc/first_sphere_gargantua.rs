//! First-Sphere Gargantua — `{4}{B}{B}` 5/4 black Phyrexian Horror.
//!
//! Oracle:
//! * When this creature enters, you draw a card and you lose 1 life.
//! * Unearth {2}{B}.
//!
//! The ETB draw-and-lose-life is a plain non-targeted triggered ability.
//!
//! GAP (keyword): "Unearth {2}{B}" — Unearth is not an available
//! `KeywordAbility` variant, and the graveyard "return with haste, exile
//! at end step" mechanic is not part of the documented effect surface.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("First-Sphere Gargantua");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_draw_lose,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_draw_lose(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: trig.controller, count: 1 },
        Effect::LoseLife { player: trig.controller, amount: 1 },
    ]
}
