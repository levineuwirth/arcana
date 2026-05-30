//! The Emperor of Palamecia // The Lord Master of Hell
//!
//! Front face (Legendary Creature — Human Noble Wizard, 2/2, {U}{R}):
//!   {T}: Add {U} or {R}. Spend this mana only to cast a noncreature spell.
//!   Whenever you cast a noncreature spell, if at least four mana was spent to cast it,
//!   put a +1/+1 counter on The Emperor of Palamecia. Then if it has three or more
//!   +1/+1 counters on it, transform it.
//!
//! Back face (Legendary Creature — Demon Noble Wizard, 5/5):
//!   Starfall — Whenever The Lord Master of Hell attacks, it deals X damage to each
//!   opponent, where X is the number of noncreature, nonland cards in your graveyard.
//!
//! GAP: {T}: Add {U} or {R} — mana ability with "spend only on noncreature spell" restriction
//!      not modeled (mana ability tap needs ActivatedAbilityDef, mana restriction not supported).
//! GAP: "if at least four mana was spent to cast it" — no mana-spent-this-cast accessor;
//!      modeled as unconditional counter placement.
//! GAP: "if it has three or more +1/+1 counters, transform" — no counter-count conditional;
//!      transform emitted unconditionally after counter (over-fires on early casts).
//! GAP: Starfall (back-face-only triggered ability) not modeled — back-face triggers are engine debt.
//! GAP: Keyword "Starfall" not in KeywordAbility enum.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Emperor of Palamecia");
    let sub_human = reg.interner_mut().intern("Human");
    let sub_noble = reg.interner_mut().intern("Noble");
    let sub_wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub_human);
    subtypes.0.insert(sub_noble);
    subtypes.0.insert(sub_wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("The Lord Master of Hell");
    let back_sub_demon = reg.interner_mut().intern("Demon");
    let back_sub_noble = reg.interner_mut().intern("Noble");
    let back_sub_wizard = reg.interner_mut().intern("Wizard");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_sub_demon);
    back_subtypes.0.insert(back_sub_noble);
    back_subtypes.0.insert(back_sub_wizard);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue() | ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![],
            ..Default::default()
        },
        spell_ability: None,
    };

    // Noncreature-spell filter for SpellCast trigger
    let noncreature_filter = ObjectFilter::new().without_types(TypeLine::CREATURE.into());

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front-face trigger: whenever you cast a noncreature spell, put a +1/+1 counter,
            // then transform. GAP: "if at least four mana spent" condition not modeled;
            // "if 3+ counters" condition not modeled — transform fires unconditionally.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(noncreature_filter),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_noncreature_cast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_noncreature_cast(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if at least four mana was spent" condition not expressible.
    // GAP: "if it has three or more +1/+1 counters, transform" condition not expressible;
    //      emitting AddCounters + Transform unconditionally.
    vec![
        Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
        Effect::Transform { target: trig.source },
    ]
}
