//! Tenth District Hero — `{1}{W}` 2/3 Human.
//! "{1}{W}, Collect evidence 2: This creature becomes a Human
//! Detective with base power and toughness 4/4 and gains vigilance."
//! "{2}{W}, Collect evidence 4: If this creature is a Detective, it
//! becomes a legendary creature named Mileva, the Stalwart, it has
//! base power and toughness 5/5, and it gains 'Other creatures you
//! control have indestructible.'"
//!
//! "Collect evidence N" (exile cards with total mana value N from your
//! graveyard) is not expressible as an ActivationCost, so only the
//! mana portion of each cost is modeled. The first ability's expressible
//! parts (SetBasePT 4/4, gains Vigilance) are emitted; becoming a
//! "Human Detective" (a subtype add) is GAP'd. The second ability's
//! "if it's a Detective" gate, the rename, and the granted anthem
//! static are not expressible, so its effect is GAP'd.

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
    let name = reg.interner_mut().intern("Tenth District Hero");
    let human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                // GAP (cost): "Collect evidence 2" (exile cards with total mv 2
                // from your graveyard) is not an expressible ActivationCost;
                // only {1}{W} is modeled.
                text: "{1}{W}, Collect evidence 2: This creature becomes a Human Detective with base power and toughness 4/4 and gains vigilance.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_detective,
            })
            .with_activated_ability(ActivatedAbilityDef {
                // GAP (cost): "Collect evidence 4" not expressible; only {2}{W}.
                text: "{2}{W}, Collect evidence 4: If this creature is a Detective, it becomes a legendary creature named Mileva, the Stalwart, base power and toughness 5/5, and gains \"Other creatures you control have indestructible.\"".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_mileva,
            }),
    )
}

fn become_detective(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "becomes a Human Detective" (adding the Detective subtype) has no
    // subtype-add Effect; the base P/T set and Vigilance grant ARE modeled.
    vec![
        Effect::SetBasePT {
            target: ctx.source,
            power: 4,
            toughness: 4,
            duration: Duration::Permanent,
        },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Vigilance,
            duration: Duration::Permanent,
        },
    ]
}

fn become_mileva(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if it's a Detective" gate, the rename to Mileva, the legendary
    // supertype add, and the granted "Other creatures you control have
    // indestructible" anthem are not expressible together.
    Vec::new()
}
