//! Beetle-Headed Merchants — `{4}{B}` 5/4 Human Citizen.
//! "Whenever this creature attacks, you may sacrifice another creature or artifact. If you do, draw a card and put a +1/+1 counter on this creature."
//! NOTE: "another" (exclude source) can't be expressed by SacrificeFilter, so the
//! sacrifice selection may include this creature itself — minor over-inclusion.

use arcana_core::actions::{OptionalPaymentKind, SacrificeFilter};
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Beetle-Headed Merchants");
    let human = reg.interner_mut().intern("Human");
    let citizen = reg.interner_mut().intern("Citizen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(citizen);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attack,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_attack(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "you may sacrifice another creature or artifact. If you do, draw a card
    // and put a +1/+1 counter on this creature." Optional Sacrifice payment; the
    // draw + counter `then` runs only after the sacrifice resolves.
    // ("another" can't exclude the source — minor over-inclusion.)
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Sacrifice(SacrificeFilter::CreatureOrArtifact),
        then: Box::new(Effect::Sequence(vec![
            Effect::DrawCards { player: trig.controller, count: 1 },
            Effect::AddCounters {
                target: trig.source,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            },
        ])),
        else_effect: None,
    }]
}
