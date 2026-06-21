//! Xathrid Gorgon — `{5}{B}` 3/6 Gorgon with Deathtouch.
//!
//! * Deathtouch
//! * `{2}{B}, {T}`: Put a petrification counter on target creature. It
//!   gains defender and becomes a colorless artifact in addition to its
//!   other types. (The "its activated abilities can't be activated"
//!   rider is GAP'd — no Effect to suppress activated abilities.)

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
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Xathrid Gorgon");
    let gorgon = reg.interner_mut().intern("Gorgon");
    let _petrification = reg.interner_mut().intern("petrification");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gorgon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{B}, {T}: Put a petrification counter on target creature. It gains defender and becomes a colorless artifact in addition to its other types.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: petrify,
            }),
    )
}

fn petrify(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let id = *id;
    let mut effects = Vec::new();
    if let Some(kind) = reg.interner().lookup("petrification").map(CounterKind::Named) {
        effects.push(Effect::AddCounters { target: id, kind, count: 1 });
    }
    effects.push(Effect::GrantKeyword {
        target: id,
        keyword: KeywordAbility::Defender,
        duration: Duration::Permanent,
    });
    effects.push(Effect::AddType {
        target: id,
        types: TypeLine::ARTIFACT.into(),
        duration: Duration::Permanent,
    });
    effects.push(Effect::SetColor {
        target: id,
        colors: ColorSet::colorless(),
        duration: Duration::Permanent,
    });
    // GAP: "Its activated abilities can't be activated." — no Effect to
    // suppress a permanent's activated abilities.
    effects
}
