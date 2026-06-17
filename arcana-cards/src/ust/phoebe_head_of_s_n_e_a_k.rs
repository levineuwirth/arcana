//! Phoebe, Head of S.N.E.A.K. — `{1}{U}{B}` 2/3 Legendary Human Spy.
//!
//! * "Phoebe can't be blocked by creatures with flavor text." GAP: no flavor-text
//!   predicate; conditional unblockable evasion not expressible.
//! * `{2}{U}{B}: Phoebe permanently steals target creature's text box.` We emit the
//!   activated ability shell (targets a creature) but GAP the text-box-steal effect —
//!   there is no Effect variant for transferring rules/flavor text.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phoebe, Head of S.N.E.A.K.");
    let human = reg.interner_mut().intern("Human");
    let spy = reg.interner_mut().intern("Spy");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(spy);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{U}{B}: Phoebe permanently steals target creature's text box.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{U}{B}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: phoebe_steal_text,
        }),
    )
}

fn phoebe_steal_text(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "permanently steals target creature's text box" — no Effect variant
    // transfers rules text / flavor text / watermarks between objects.
    Vec::new()
}
