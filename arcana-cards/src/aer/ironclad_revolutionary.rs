//! Ironclad Revolutionary — `{4}{B}{B}` 4/4 black Aetherborn Artificer.
//! "When this creature enters, you may sacrifice an artifact. If you do, put two
//! +1/+1 counters on this creature and each opponent loses 2 life."
//! GAP: "optional sacrifice conditional" not directly in catalog; emitting sacrifice +
//! counters + life loss sequence (best-effort, no conditionality).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ironclad Revolutionary");
    let aetherborn = reg.interner_mut().intern("Aetherborn");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aetherborn);
    subtypes.0.insert(artificer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_sacrifice_and_grow,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_sacrifice_and_grow(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![
        Effect::Sacrifice {
            player: trig.controller,
            filter: ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
            count: 1,
        },
        Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 2,
        },
    ];
    for p in script::opponents(state, trig.controller) {
        effects.push(Effect::LoseLife { player: p, amount: 2 });
    }
    effects
}
