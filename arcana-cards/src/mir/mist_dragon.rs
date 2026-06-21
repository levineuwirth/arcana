//! Mist Dragon — `{4}{U}{U}` 4/4 Dragon.
//! {0}: This creature gains flying. (Lasts indefinitely.)
//! {0}: This creature loses flying. (Lasts indefinitely.)
//! {3}{U}{U}: This creature phases out.
//!
//! No keyword line. Ability 1 grants flying permanently (GrantKeyword
//! with Duration::Permanent). Ability 2 ("loses flying") has no single-
//! keyword removal primitive (LoseAllAbilities would strip everything),
//! so it is GAP'd. Ability 3 ("phases out") has no phasing primitive and
//! is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mist Dragon");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // {0}: This creature gains flying (indefinitely).
            .with_activated_ability(ActivatedAbilityDef {
                text: "{0}: This creature gains flying.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{0}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_flying,
            }),
        // GAP: "{0}: This creature loses flying." — no single-keyword removal
        // primitive (LoseAllAbilities strips ALL abilities, which is wrong).
        // GAP: "{3}{U}{U}: This creature phases out." — no phasing primitive.
    )
}

fn gain_flying(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GrantKeyword {
        target: ctx.source,
        keyword: KeywordAbility::Flying,
        duration: Duration::Permanent,
    }]
}
