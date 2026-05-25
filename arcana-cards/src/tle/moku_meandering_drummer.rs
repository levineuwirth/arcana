//! Moku, Meandering Drummer — {1}{R} 2/2 Legendary Creature — Human Bard Ally.
//! "Whenever you cast a noncreature spell, you may pay {1}. If you do,
//! Moku gets +2/+1 and creatures you control gain haste until end of turn."
//!
//! GAP: the "you may pay {1}" optional-cost gate is not expressible with the
//! demonstrated effect catalog (no optional-cost variant on triggered
//! resolution). The conditional payload (pump + haste sweep) is emitted as
//! best-effort so the rest of the rules text is captured; verify will flag the
//! missing cost gate.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Moku, Meandering Drummer");
    let human = reg.interner_mut().intern("Human");
    let bard = reg.interner_mut().intern("Bard");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(bard);
    subtypes.0.insert(ally);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(
                    ObjectFilter::new().without_types(TypeLine::CREATURE.into()),
                ),
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

/// "If you do, Moku gets +2/+1 and creatures you control gain haste until end
/// of turn." (Optional `{1}` cost is GAPped — see file-level note.)
fn on_noncreature_cast(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may pay {1}" — optional triggered-ability cost cannot be
    // modeled with the catalog; resolving the payload unconditionally.
    let creatures_you_control = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    vec![
        Effect::Pump {
            target: trig.source,
            power: 2,
            toughness: 1,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::ForEach {
            targets: creatures_you_control,
            effect: Box::new(Effect::GrantKeyword {
                target: arcana_core::objects::NULL_OBJECT_ID,
                keyword: KeywordAbility::Haste,
                duration: Duration::EndOfTurn,
            }),
        },
    ]
}
