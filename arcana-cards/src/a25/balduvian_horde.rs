//! Balduvian Horde — `{2}{R}{R}` 5/5 red Human Barbarian.
//! "When this creature enters, sacrifice it unless you discard a card at random."
//! ETB trigger: OptionalPayment gate — pay = discard a card (decline = sacrifice
//! this creature). Fidelity note: "at random" is approximated as a chooser-picked
//! discard (OptionalPaymentKind::Discard offers no random-selection variant).

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::targets::ObjectFilter;
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
    let name = reg.interner_mut().intern("Balduvian Horde");
    let human = reg.interner_mut().intern("Human");
    let barbarian = reg.interner_mut().intern("Barbarian");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(barbarian);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_unless_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_unless_discard(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // "Sacrifice it unless you discard a card." Pay = discard 1 (nothing else);
    // decline = sacrifice this creature. (Fidelity: "at random" approximated as a
    // chooser-picked discard — Discard offers no random-selection variant.)
    let nm = reg.interner().lookup("Balduvian Horde");
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
