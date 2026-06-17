//! Fell Stinger — `{2}{B}` 3/2 Zombie Scorpion with Deathtouch.
//! Exploit: ETB sacrifice a creature, then target player draws two cards and
//! loses 2 life. Modeled as one ETB trigger (the "may" / reflexive
//! exploit-linkage is approximated as mandatory — see GAP).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fell Stinger");
    let zombie = reg.interner_mut().intern("Zombie");
    let scorpion = reg.interner_mut().intern("Scorpion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(scorpion);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            // GAP: Exploit's "you MAY sacrifice a creature" and the reflexive
            // "when this creature exploits a creature" linkage are not
            // separately expressible; modeled as a single ETB that sacrifices
            // a creature then grants the payoff to target player.
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: exploit,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_player()],
        }),
    )
}

fn exploit(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Sequence(vec![
        Effect::Sacrifice {
            player: trig.controller,
            filter: ObjectFilter::creature(),
            count: 1,
        },
        Effect::DrawCards { player: *p, count: 2 },
        Effect::LoseLife { player: *p, amount: 2 },
    ])]
}
