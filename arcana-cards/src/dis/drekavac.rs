//! Drekavac — `{1}{B}` 3/3 black creature. "When this creature enters,
//! sacrifice it unless you discard a noncreature card." Wired as an
//! OptionalPayment (pay = discard 1, decline = sacrifice this creature).
//! Fidelity note: the payment can't enforce "noncreature" — Discard(N) takes
//! no filter — so the discarded card isn't constrained to noncreature.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::targets::ObjectFilter;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drekavac");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_cost,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_cost(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // "Sacrifice it unless you discard a noncreature card." Pay = discard 1 (the
    // noncreature constraint is not enforced — Discard takes no filter); decline =
    // sacrifice this creature.
    let nm = reg.interner().lookup("Drekavac");
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Discard(1),
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: Some(Box::new(Effect::Sacrifice {
            player: trig.controller,
            filter: ObjectFilter { name: nm, ..ObjectFilter::default() },
            count: 1,
        })),
    }]
}
