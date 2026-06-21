//! Varragoth, Bloodsky Sire — `{2}{B}` 2/3 Legendary Demon Rogue with
//! Deathtouch.
//!
//! * Deathtouch — keyword line.
//! * "Boast — {1}{B}: Target player searches their library for a card,
//!   then shuffles and puts that card on top." → an activated ability
//!   with mana cost {1}{B}; Boast's "only if this creature attacked this
//!   turn" gate is `ActivationCost::activation_condition` +
//!   `conditions::source_attacked_this_turn`, "only once each turn" is
//!   `once_per_turn`. The effect is a library search by the TARGET player
//!   into their own library (`Effect::Search`, destination = that
//!   player's library, shuffle implied). Fidelity GAP: `Zone::Library`
//!   has no top/bottom distinction, so "puts that card on top" lands the
//!   card in the library generically rather than guaranteed on top.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Varragoth, Bloodsky Sire");
    let demon = reg.interner_mut().intern("Demon");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Boast — {1}{B}: Target player searches their library for a card, then shuffles and puts that card on top.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B}").unwrap(),
                    activation_condition: Some(|s, src, _you, _reg| {
                        arcana_core::conditions::source_attacked_this_turn(s, src)
                    }),
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: boast_search,
            }),
    )
}

fn boast_search(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    vec![Effect::Search {
        player: *p,
        zone: Zone::Library(*p),
        filter: ObjectFilter::default(),
        destination: Zone::Library(*p),
        reveal: false,
    }]
}
