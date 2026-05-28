//! Illuminor Szeras — `{2}{B}` 3/3 Legendary Artifact Creature — Necron.
//! Secrets of the Soul — `{T}, Sacrifice another creature: Add {B} equal to the sacrificed creature's mana value.`
//! GAP: ActivationCost::sacrifice is self-only; "sacrifice another creature" not expressible.
//! GAP: "amount equal to sacrificed creature's mana value" — can't read mana value of just-sacrificed permanent.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Illuminor Szeras");
    let necron = reg.interner_mut().intern("Necron");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(necron);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Sacrifice another creature: Add {B} equal to the sacrificed creature's mana value.".into(),
                cost: ActivationCost { tap: true, ..ActivationCost::default() },
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: soul_harvest,
            }),
    )
}

fn soul_harvest(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "add {B} equal to sacrificed creature's mana value" — can't read CMC of sacrificed permanent
    // Approximating as 1 black mana
    vec![Effect::AddMana { player: ctx.controller, mana: vec![ManaUnit::plain(ManaColor::Black, ctx.source)] }]
}
