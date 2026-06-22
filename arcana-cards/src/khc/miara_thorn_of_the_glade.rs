//! Miara, Thorn of the Glade — `{1}{B}` 1/2 Legendary Elf Scout.
//! "Whenever Miara or another Elf you control dies, you may pay {1} and
//! 1 life. If you do, draw a card." Partner.
//!
//! The death trigger watches Miara herself OR any other Elf you control
//! moving battlefield→graveyard. Because Miara's own death also satisfies
//! the watched event, this is an "any matching" death trigger (the
//! filter is Elf creatures you control, and Miara is one). The "you may
//! pay {1} and 1 life" gate is a single OptionalPayment — but the cost
//! combines mana AND life, and OptionalPaymentKind in v1 carries ONLY
//! one of Mana or Life, so the combined cost is a fidelity GAP: we model
//! the mana side (the more common gate) and draw on payment.
//! Partner is GAP (no KeywordAbility variant; Commander-only deck rule).

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Miara, Thorn of the Glade");
    let elf = reg.interner_mut().intern("Elf");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(scout);

    let elf_filter = script_elf_filter(reg);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: keyword — Partner (Commander deck-construction rule; no KeywordAbility variant).
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: elf_filter,
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: may_pay_then_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn script_elf_filter(reg: &mut CardRegistry) -> ObjectFilter {
    let elf = reg.interner_mut().intern("Elf");
    ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(vec![elf])
}

fn may_pay_then_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: combined cost — oracle is "pay {1} AND 1 life"; OptionalPaymentKind
    // carries only one of Mana/Life in v1, so we model the {1} mana gate.
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{1}").expect("valid cost")),
        then: Box::new(Effect::DrawCards { player: trig.controller, count: 1 }),
        else_effect: None,
    }]
}
