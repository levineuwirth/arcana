//! Poxwalkers — `{2}{B}` 3/1 black Zombie with Deathtouch.
//! "Curse of the Walking Pox — Whenever you cast a spell from anywhere other
//! than your hand, return this card from your graveyard to the battlefield
//! tapped."
//!
//! "Curse of the Walking Pox" is an ability word (flavor label), not a
//! keyword, so it carries no mechanical weight.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Poxwalkers");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP (zone restriction): "from anywhere other than your hand"
                // has no expressible source-zone filter on SpellCast, so this
                // fires on every spell you cast — a documented over-fire. The
                // ability is zoned to the graveyard so it can return this card.
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: return_self_to_battlefield,
                trigger_zones: vec![Zone::Graveyard(0)],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn return_self_to_battlefield(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP (tapped): ReturnFromGraveyardToBattlefield has no "tapped" flag, so
    // the card returns untapped — a minor fidelity gap.
    vec![Effect::ReturnFromGraveyardToBattlefield { target: trig.source }]
}
