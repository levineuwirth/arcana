//! Soraya the Falconer — `{1}{W}{W}` 2/2 Legendary Human.
//!
//! * "Bird creatures get +1/+1." — a static continuous anthem with no
//!   expressible primitive in this card class; GAP'd.
//! * "{1}{W}: Target Bird creature gains banding until end of turn." —
//!   an activated ability granting the Banding keyword to a targeted
//!   Bird.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soraya the Falconer");
    let human = reg.interner_mut().intern("Human");
    // Intern "Bird" so the target filter resolves to a real subtype symbol.
    let _bird = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);

    let bird_filter = arcana_core::script::subtype_filter(reg, "Bird");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: static "Bird creatures get +1/+1" anthem — no continuous
        // type-filtered pump primitive for this card class.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{W}: Target Bird creature gains banding until end of turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{W}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(bird_filter),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: grant_banding,
        }),
    )
}

fn grant_banding(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::GrantKeyword {
        target: *id,
        keyword: KeywordAbility::Banding,
        duration: Duration::EndOfTurn,
    }]
}
