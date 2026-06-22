//! Hiveheart Shaman — `{3}{G}` 3/5 Creature — Human Shaman.
//!
//! Oracle:
//! * Whenever this creature attacks, you may search your library for a basic
//!   land card that doesn't share a land type with a land you control, put that
//!   card onto the battlefield, then shuffle.
//! * {5}{G}: Create a 1/1 green Insect creature token. Put X +1/+1 counters on
//!   it, where X is the number of basic land types among lands you control.
//!   Activate only as a sorcery.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hiveheart Shaman");
    let human = reg.interner_mut().intern("Human");
    let shaman = reg.interner_mut().intern("Shaman");
    let _insect = reg.interner_mut().intern("Insect");
    // Intern the five basic land subtypes so the resolver lookups succeed.
    let _plains = reg.interner_mut().intern("Plains");
    let _island = reg.interner_mut().intern("Island");
    let _swamp = reg.interner_mut().intern("Swamp");
    let _mountain = reg.interner_mut().intern("Mountain");
    let _forest = reg.interner_mut().intern("Forest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_tutor_basic,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}{G}: Create a 1/1 green Insect creature token. Put X +1/+1 counters on it, where X is the number of basic land types among lands you control. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_insect,
            }),
    )
}

fn attack_tutor_basic(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: the "that doesn't share a land type with a land you control"
    // restriction is not expressible as an ObjectFilter; tutor a basic land
    // onto the battlefield (the faithful core of the effect).
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter: ObjectFilter::new()
            .with_types(TypeLine::LAND.into())
            .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC)),
        tapped: false,
    }]
}

fn basic_land_types(state: &GameState, reg: &CardRegistry, you: arcana_core::types::PlayerId) -> u32 {
    let mut n = 0;
    for ty in ["Plains", "Island", "Swamp", "Mountain", "Forest"] {
        let filter = script::subtype_filter(reg, ty)
            .with_types(TypeLine::LAND.into())
            .controlled_by(ControllerConstraint::You);
        if script::count_matching(state, &filter, you) > 0 {
            n += 1;
        }
    }
    n
}

fn make_insect(state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let insect = reg.interner().lookup("Insect").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    let x = basic_land_types(state, reg, ctx.controller);
    let token = TokenDefinition {
        name: insect,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: the +1/+1 counters target the freshly-minted token, whose id is not
    // visible to the resolver; X is computed faithfully but the counter
    // placement onto the new token is not expressible here.
    let _ = x;
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token,
    }]
}
