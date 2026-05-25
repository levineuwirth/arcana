//! Powerstone Engineer — `{1}{W}` 2/1 Human Artificer. "When this
//! creature dies, create a tapped Powerstone token." (Powerstone is
//! a colorless artifact token with `{T}: Add {C}`-restricted-mana.)
//! The death trigger emits a Powerstone token; the "enters tapped"
//! rider and the Powerstone activated ability are deferred engine
//! work — see GAPs in the resolver.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Powerstone Engineer");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    // Pre-intern the token subtype so the resolver can look it up
    // through the immutable interner at resolve time.
    let _powerstone = reg.interner_mut().intern("Powerstone");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: create_powerstone_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Death trigger resolution: create a colorless Powerstone artifact
/// token for this creature's controller.
fn create_powerstone_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let powerstone = reg
        .interner()
        .lookup("Powerstone")
        .expect("Powerstone interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(powerstone);
    // GAP: token should enter the battlefield TAPPED — no
    // `CreateTokenTapped` / "enters tapped" rider is available in the
    // effect catalog. The Powerstone is created untapped.
    // GAP: Powerstone's `{T}: Add {C}` (restricted to artifact spells
    // / activated abilities) is deferred engine work — recognized by
    // subtype, not authored here.
    let token = TokenDefinition {
        name: powerstone,
        colors: ColorSet::colorless(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
