//! Tifa, Martial Artist — `{1}{R}{G}{W}` 4/4 Legendary Human Monk.
//!
//! Melee. (GAP — Melee is not in the usable KeywordAbility surface for
//! this card class.)
//! Whenever one or more creatures you control with power 7 or greater
//! deal combat damage to a player, untap all creatures you control. If
//! it's the first combat phase of your turn, there is an additional
//! combat phase after this phase. (The extra-combat-phase rider is
//! GAP'd; the untap is expressed.)

use arcana_core::effects::Effect;
use arcana_core::objects::NULL_OBJECT_ID;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tifa, Martial Artist");
    let human = reg.interner_mut().intern("Human");
    let monk = reg.interner_mut().intern("Monk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(monk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: Melee is not an available KeywordAbility variant for this
        // card class; emit no keyword for it.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .with_min_power(7),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: untap_all_your_creatures,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "Untap all creatures you control." (The "additional combat phase"
/// rider is GAP'd — there is no extra-combat-phase Effect primitive.)
fn untap_all_your_creatures(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    // GAP: "If it's the first combat phase of your turn, there is an
    // additional combat phase after this phase" — no extra-combat-phase
    // Effect primitive is available.
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Untap { target: NULL_OBJECT_ID }),
    }]
}
