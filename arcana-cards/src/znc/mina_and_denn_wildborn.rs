//! Mina and Denn, Wildborn — `{2}{R}{G}` 4/4 Legendary Elf Ally.
//! "You may play an additional land on each of your turns.
//!  {R}{G}, Return a land you control to its owner's hand: Target creature gains
//!  trample until end of turn."
//!
//! The "additional land" static permission has no primitive — GAP'd. The
//! activated ability's grant-trample effect is expressed; its "Return a land
//! you control to its owner's hand" component cost has no ActivationCost field,
//! so the cost is GAP'd (only the {R}{G} mana cost is enforced).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mina and Denn, Wildborn");
    let elf = reg.interner_mut().intern("Elf");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(ally);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: static "You may play an additional land on each of your turns" — no
    // extra-land-play permission primitive.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{R}{G}, Return a land you control to its owner's hand: Target creature gains trample until end of turn.".into(),
            // GAP: "Return a land you control to its owner's hand" component cost
            // has no ActivationCost field; only the {R}{G} mana cost is enforced.
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{R}{G}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: grant_trample,
        }),
    )
}

fn grant_trample(
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
        keyword: KeywordAbility::Trample,
        duration: Duration::EndOfTurn,
    }]
}
