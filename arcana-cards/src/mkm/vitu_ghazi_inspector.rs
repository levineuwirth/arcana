//! Vitu-Ghazi Inspector — `{1}{G}` 1/3 Elf Detective with Reach.
//! "As an additional cost … you may collect evidence 6." (cast-time cost — GAP).
//! "When this creature enters, if evidence was collected, put a +1/+1 counter on
//! target creature and you gain 2 life."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vitu-Ghazi Inspector");
    let elf = reg.interner_mut().intern("Elf");
    let detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(detective);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    // GAP: "collect evidence 6" additional cast cost — no cast-time exile-from-graveyard
    // cost field; not expressible.
    // GAP (intervening-if): "if evidence was collected" — no predicate for the collect-
    // evidence flag, so the ETB resolves unconditionally (closest expressible form).
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: counter_and_gain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            }),
    )
}

fn counter_and_gain(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    if let Some(TargetChoice::Object(id)) = trig.targets.targets.first() {
        effects.push(Effect::AddCounters {
            target: *id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        });
    }
    effects.push(Effect::GainLife { player: trig.controller, amount: 2 });
    effects
}
