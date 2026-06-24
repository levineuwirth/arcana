//! Queen Allenal of Ruadach — `{G}{W}{W}` */* Legendary Elf Noble.
//! Queen Allenal of Ruadach's power and toughness are each equal to the
//! number of creatures you control.
//! If one or more creature tokens would be created under your control,
//! those tokens plus a 1/1 white Soldier creature token are created
//! instead.
//!
//!  * The CDA "power and toughness equal to the number of creatures you
//!    control" is wired via a self-CDA installed on ETB
//!    (ContinuousEffect::self_pt_from_match at Layer 7a).
//!  * GAP: the token-creation replacement ("those tokens plus a Soldier are
//!    created instead") has no available replacement primitive.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
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
    let name = reg.interner_mut().intern("Queen Allenal of Ruadach");
    let elf = reg.interner_mut().intern("Elf");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(noble);

    // GAP: creature-token-creation replacement (extra 1/1 Soldier).
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{W}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: install_cda,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// Layer 7a self-CDA: P/T each equal to the number of creatures you control.
fn install_cda(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_from_match(
            trig.source,
            filter,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
