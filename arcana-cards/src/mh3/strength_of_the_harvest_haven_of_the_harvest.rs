//! Strength of the Harvest // Haven of the Harvest — `{2}{G/W}` enchantment —
//! Aura // Land (a modal double-faced card).
//! Front: "Enchant creature. Enchanted creature gets +1/+1 for each creature
//!  and/or enchantment you control."
//! Back: "Haven of the Harvest — Land. This land enters tapped. {T}: Add {G}
//!  or {W}."
//!
//! The Aura front is modeled fully: an ETB-installed `attached_pt_dynamic`
//! whose compute fn counts creatures-you-control plus enchantments-you-control
//! (type-only filters, expressible) and returns (X, X). The MDFC Land back
//! face has no demonstrated alternate-face API here, so it is GAP'd (front
//! face only).

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Strength of the Harvest");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G/W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        // GAP: MDFC Land back face (Haven of the Harvest) — no demonstrated
        // alternate-face API; only the Aura front face is modeled.
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt_dynamic(
            trig.source,
            creatures_and_enchantments_you_control,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn creatures_and_enchantments_you_control(
    state: &GameState,
    source: ObjectId,
) -> (i32, i32) {
    let Some(controller) = state.object_or_lki(source).map(|o| o.controller) else {
        return (0, 0);
    };
    let creatures = script::count_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        controller,
    );
    let enchantments = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::ENCHANTMENT.into())
            .controlled_by(ControllerConstraint::You),
        controller,
    );
    let n = (creatures + enchantments) as i32;
    (n, n)
}
