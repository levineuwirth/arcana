//! Merieke Ri Berit — `{W}{U}{B}` 1/1 legendary Human.
//! "Merieke Ri Berit doesn't untap during your untap step."
//! "{T}: Gain control of target creature for as long as you control
//! Merieke Ri Berit. When Merieke Ri Berit leaves the battlefield or
//! becomes untapped, destroy that creature. It can't be regenerated."
//!
//! The "doesn't untap" line is a static untap-restriction with no
//! expressible primitive — GAP'd. The {T} activated ability gains
//! control of the target creature (Effect::ChangeControl); the linked
//! "for as long as / destroy when leaves-or-untaps / can't be
//! regenerated" rider has no expressible control-linkage primitive in
//! the catalog, so that portion is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Merieke Ri Berit");
    let human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);

    // GAP: static "doesn't untap during your untap step".
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Gain control of target creature for as long as you control Merieke Ri Berit. When Merieke Ri Berit leaves the battlefield or becomes untapped, destroy that creature. It can't be regenerated.".into(),
            cost: ActivationCost::tap_only(),
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: gain_control,
        }),
    )
}

fn gain_control(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "for as long as you control ~ / destroy when ~ leaves or
    // becomes untapped / can't be regenerated" linkage is not expressible;
    // only the gain-control portion is modeled.
    vec![Effect::ChangeControl {
        target: *id,
        new_controller: ctx.controller,
    }]
}
