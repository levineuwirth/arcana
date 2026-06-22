//! Mutalith Vortex Beast — `{4}{U}{R}` 6/6 Mutant Beast with Trample.
//!
//! Oracle:
//! * Trample.
//! * Warp Vortex — When this creature enters, flip a coin for each
//!   opponent you have. For each flip you win, draw a card. For each
//!   flip you lose, this creature deals 3 damage to that player.
//!
//! "Warp Vortex" is an ability word (flavor name), not a keyword. The
//! ETB builds one coin flip per opponent: win → draw a card, lose →
//! 3 damage to that opponent.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mutalith Vortex Beast");
    let mutant = reg.interner_mut().intern("Mutant");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutant);
    subtypes.0.insert(beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: warp_vortex,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn warp_vortex(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let opponents = script::opponents(state, trig.controller);
    let flips: Vec<Effect> = opponents
        .into_iter()
        .map(|opp| Effect::FlipCoin {
            player: trig.controller,
            win: Box::new(Effect::DrawCards {
                player: trig.controller,
                count: 1,
            }),
            lose: Some(Box::new(Effect::DealDamage {
                source: trig.source,
                target: DamageTarget::Player(opp),
                amount: 3,
            })),
        })
        .collect();
    vec![Effect::Sequence(flips)]
}
