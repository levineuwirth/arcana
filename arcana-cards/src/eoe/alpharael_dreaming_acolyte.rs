//! Alpharael, Dreaming Acolyte — `{1}{U}{B}` 2/3 Legendary Human Cleric.
//!
//! When Alpharael enters, draw two cards. Then discard two cards unless
//! you discard an artifact card.
//! "During your turn, Alpharael has deathtouch." (static — GAP'd)
//!
//! The ETB draws two and discards two. The "unless you discard an
//! artifact card" alternative (discard one artifact instead of two) has
//! no demonstrated branch-on-discarded-type primitive, so it is a
//! documented fidelity GAP (two cards are discarded). The during-your-
//! turn deathtouch is a conditional static and is GAP'd.

use arcana_core::effects::{DiscardChoice, Effect};
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
    let name = reg.interner_mut().intern("Alpharael, Dreaming Acolyte");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP (static): "During your turn, Alpharael has deathtouch" — conditional continuous ability.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: draw_then_discard,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn draw_then_discard(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP (fidelity): "unless you discard an artifact card" alternative has no branch-on-discarded-type primitive.
    vec![Effect::Sequence(vec![
        Effect::DrawCards {
            player: trig.controller,
            count: 2,
        },
        Effect::Discard {
            player: trig.controller,
            count: 2,
            choice: DiscardChoice::ControllerChooses,
        },
    ])]
}
