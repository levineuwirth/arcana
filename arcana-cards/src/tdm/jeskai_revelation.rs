//! Jeskai Revelation — `{4}{U}{R}{W}` instant. "Return target spell
//! or permanent to its owner's hand. Jeskai Revelation deals 4 damage
//! to any target. Create two 1/1 white Monk creature tokens with
//! prowess. Draw two cards. You gain 4 life."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, ObjectOrPlayer, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jeskai Revelation");
    let _monk = reg.interner_mut().intern("Monk");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue() | ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return target spell or permanent to its owner's hand. \
                       Jeskai Revelation deals 4 damage to any target. Create \
                       two 1/1 white Monk creature tokens with prowess. Draw \
                       two cards. You gain 4 life.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(ObjectFilter::default()),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement::any_target(),
                ],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let monk = reg.interner().lookup("Monk").expect("Monk interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(monk);
    // GAP: Monk tokens specified with prowess; prowess is not an
    // expressible keyword, so the tokens are created without it.
    let token = TokenDefinition {
        name: monk,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    let mut effects = Vec::new();
    if let Some(TargetChoice::Object(id)) = entry.targets.targets.first() {
        effects.push(Effect::ReturnToHand { target: *id });
    }
    if let Some(t) = entry.targets.targets.get(1) {
        let dt = match t {
            TargetChoice::Object(id) => DamageTarget::Object(*id),
            TargetChoice::Player(p) => DamageTarget::Player(*p),
            TargetChoice::ObjectOrPlayer(o) => match o {
                ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
                ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
            },
        };
        effects.push(Effect::DealDamage { source: entry.source, target: dt, amount: 4 });
    }
    effects.push(Effect::CreateToken { controller: entry.controller, token: token.clone() });
    effects.push(Effect::CreateToken { controller: entry.controller, token });
    effects.push(Effect::DrawCards { player: entry.controller, count: 2 });
    effects.push(Effect::GainLife { player: entry.controller, amount: 4 });
    effects
}
